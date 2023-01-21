#[doc = "Register `cpu_core_cfg10` reader"]
pub struct R(crate::R<CPU_CORE_CFG10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CORE_CFG10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CORE_CFG10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CORE_CFG10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_core_cfg10` writer"]
pub struct W(crate::W<CPU_CORE_CFG10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CORE_CFG10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CPU_CORE_CFG10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CORE_CFG10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pico_rtc_cnt_h` reader - "]
pub type PICO_RTC_CNT_H_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pico_rtc_cnt_h(&self) -> PICO_RTC_CNT_H_R {
        PICO_RTC_CNT_H_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_core_cfg10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_core_cfg10](index.html) module"]
pub struct CPU_CORE_CFG10_SPEC;
impl crate::RegisterSpec for CPU_CORE_CFG10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_core_cfg10::R](R) reader structure"]
impl crate::Readable for CPU_CORE_CFG10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_core_cfg10::W](W) writer structure"]
impl crate::Writable for CPU_CORE_CFG10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_core_cfg10 to value 0"]
impl crate::Resettable for CPU_CORE_CFG10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
